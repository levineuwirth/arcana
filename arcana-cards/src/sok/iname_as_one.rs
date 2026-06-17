//! Iname as One — `{8}{B}{B}{G}{G}` 8/8 Legendary Creature — Spirit.
//! When Iname as One enters, if you cast it from your hand, you may search
//! your library for a Spirit permanent card, put it onto the battlefield,
//! then shuffle.
//! When Iname as One dies, you may exile it. If you do, return target
//! Spirit permanent card from your graveyard to the battlefield.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iname as One");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let spirit_in_gy = script::subtype_filter(reg, "Spirit");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{B}{B}{G}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                // GAP: "if you cast it from your hand" gate — no condition
                // predicate exposes how the source entered; tutor fires
                // unconditionally as the closest expressible form.
                effect: etb_tutor_spirit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: spirit_in_gy,
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_tutor_spirit(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: script::subtype_filter(reg, "Spirit"),
        tapped: false,
    }]
}

fn dies_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    let Some(target) = trig.targets.targets.first() else {
        return vec![Effect::ExileFromGraveyard { target: id }];
    };
    let TargetChoice::Object(tid) = target else {
        return vec![Effect::ExileFromGraveyard { target: id }];
    };
    vec![
        Effect::ExileFromGraveyard { target: id },
        Effect::ReturnFromGraveyardToBattlefield { target: *tid },
    ]
}
