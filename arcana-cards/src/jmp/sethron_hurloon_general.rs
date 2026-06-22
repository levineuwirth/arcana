//! Sethron, Hurloon General — `{3}{R}{R}` 4/4 Legendary Creature — Minotaur
//! Warrior (red).
//!
//! * "Whenever Sethron or another nontoken Minotaur you control enters,
//!   create a 2/3 red Minotaur creature token." — a `ZoneChange`-to-battlefield
//!   trigger filtered to nontoken Minotaurs you control (the filter matches
//!   Sethron itself as well as other Minotaurs).
//! * "{2}{B/R}: Minotaurs you control get +1/+0 and gain menace and haste
//!   until end of turn." — an activated ability that pumps + keyword-grants
//!   each Minotaur you control via a `ForEach`-style `Sequence`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sethron, Hurloon General");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let minotaur_filter = ObjectFilter::new()
        .with_subtype_sym(minotaur)
        .controlled_by(ControllerConstraint::You)
        .nontoken();

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: minotaur_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: make_minotaur_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B/R}: Minotaurs you control get +1/+0 and gain menace and haste until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B/R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_minotaurs,
            }),
    )
}

fn make_minotaur_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let minotaur = reg.interner().lookup("Minotaur").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: arcana_core::effects::TokenDefinition {
            name: minotaur,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn pump_minotaurs(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let minotaur = reg.interner().lookup("Minotaur").unwrap_or_default();
    let filter = ObjectFilter::new()
        .with_subtype_sym(minotaur)
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    let effects = ids
        .into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Menace, KeywordAbility::Haste],
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
