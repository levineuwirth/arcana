//! Atla Palani, Nest Tender — `{1}{R}{G}{W}` 2/3 Legendary Human Shaman.
//! "{2}, {T}: Create a 0/1 green Egg creature token with defender.
//!  Whenever an Egg you control dies, reveal cards from the top of your library
//!  until you reveal a creature card. Put that card onto the battlefield and
//!  the rest on the bottom of your library in a random order."

use arcana_core::effects::{Effect, KeywordAbility, RevealDest, DigRest, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Atla Palani, Nest Tender");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    reg.interner_mut().intern("Egg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let egg_filter = script::subtype_filter(reg, "Egg")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}: Create a 0/1 green Egg creature token with defender.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_egg,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: egg_filter,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: hatch_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_egg(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let egg = reg.interner().lookup("Egg").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(egg);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: egg,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Defender],
            abilities: vec![],
        },
    }]
}

fn hatch_creature(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: None,
    }]
}
