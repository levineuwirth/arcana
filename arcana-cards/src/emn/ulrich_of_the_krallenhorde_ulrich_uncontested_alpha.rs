//! Ulrich of the Krallenhorde // Ulrich, Uncontested Alpha
//! `{3}{R}{G}` Legendary Creature — Human Werewolf 4/4
//!
//! Front: Whenever this creature enters or transforms into Ulrich of the
//! Krallenhorde, target creature gets +4/+4 until end of turn.
//! At the beginning of each upkeep, if no spells were cast last turn,
//! transform Ulrich of the Krallenhorde.
//!
//! Back (Legendary Creature — Werewolf 6/6): Whenever this creature
//! transforms into Ulrich, Uncontested Alpha, you may have it fight
//! target non-Werewolf creature you don't control.
//! At the beginning of each upkeep, if a player cast two or more spells
//! last turn, transform Ulrich.
//!
//! GAP: "enters or transforms into" — no TransformsInto TriggerCondition; using
//!      ZoneChange (ETB only) for the +4/+4 pump trigger. Transforms-into path
//!      is not authored.
//! GAP: "at the beginning of each upkeep" — no StepBegins TriggerCondition shown;
//!      upkeep-based werewolf transform triggers not authored.
//! GAP: "if no spells were cast last turn" / "if a player cast two or more spells
//!      last turn" — last-turn spell count not queryable; condition not expressible.
//! GAP: back-face-only triggered abilities (fight trigger, back upkeep trigger)
//!      not modeled per engine limitations.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ulrich of the Krallenhorde");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // Back face: Ulrich, Uncontested Alpha
    let back_name = reg.interner_mut().intern("Ulrich, Uncontested Alpha");
    let werewolf_sub2 = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub2);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: None,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face ETB pump trigger (partial — ETB only, not "transforms into")
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: front_pump_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
        // GAP: "transforms into Ulrich of the Krallenhorde" path for +4/+4 not modeled
        //      (no TransformsInto TriggerCondition).
        // GAP: upkeep-based werewolf transform triggers not authored (no StepBegins
        //      TriggerCondition; "no spells cast last turn" condition not expressible).
        // GAP: back-face-only triggered ability (fight on transforms-into-back) not
        //      modeled per engine limitations.
    )
}

fn front_pump_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 4,
        toughness: 4,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
