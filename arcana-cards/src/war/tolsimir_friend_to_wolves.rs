//! Tolsimir, Friend to Wolves — `{2}{G}{G}{W}` 3/3 Legendary Elf Scout.
//!
//! * When Tolsimir enters, create Voja, Friend to Elves, a legendary 3/3
//!   green and white Wolf creature token. (// GAP: token's custom name
//!   "Voja" and its Legendary supertype are fidelity gaps — modeled as a
//!   plain 3/3 green-and-white Wolf token.)
//! * Whenever a Wolf you control enters, you gain 3 life and that creature
//!   fights up to one target creature you don't control.
//!
//! "Fight" on the Scryfall keyword line is a flavor/ability indicator, not a
//! `KeywordAbility` variant → `keywords: vec![]`.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tolsimir, Friend to Wolves");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_voja,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: script::subtype_filter(reg, "Wolf")
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: wolf_enters_gain_and_fight,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_make_voja(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token's custom name "Voja, Friend to Elves" and Legendary supertype
    // are fidelity gaps; modeled as a plain 3/3 green-and-white Wolf token.
    let wolf = reg.interner().lookup("Wolf").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: wolf,
            colors: ColorSet::green() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn wolf_enters_gain_and_fight(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::GainLife {
        player: trig.controller,
        amount: 3,
    }];
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        let wolf = trig.entering_object().unwrap_or(trig.source);
        effects.push(Effect::Fight { a: wolf, b: *id });
    }
    effects
}
