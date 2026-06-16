//! Pinnacle Monk // Mystic Peak — MDFC
//!
//! Front: {3}{R}{R} Creature — Djinn Monk 2/2
//!   Prowess (Whenever you cast a noncreature spell, this creature gets +1/+1 until end of turn.)
//!   When this creature enters, return target instant or sorcery card from your graveyard to your hand.
//! Back: Mystic Peak — Land
//!   As this land enters, you may pay 3 life. If you don't, it enters tapped.
//!   {T}: Add {R}.
//!
//! Prowess is not a usable KeywordAbility variant, so it is wired as a SpellCast trigger
//! (noncreature spell cast by you -> +1/+1 EOT on self) rather than a keyword.
//! GAP: the back land's "pay 3 life or enter tapped" ETB choice and its "{T}: Add {R}" mana
//! ability are not modeled (land MDFC back-face activations are engine debt).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pinnacle Monk");
    let djinn = reg.interner_mut().intern("Djinn");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Mystic Peak — Land
    let back_name = reg.interner_mut().intern("Mystic Peak");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::LAND.into(),
            // GAP: "pay 3 life or enters tapped" and "{T}: Add {R}" not modeled for land MDFC back.
            ..Default::default()
        },
        spell_ability: None,
    };

    let instant_or_sorcery_in_gy = TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::new()
                .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
        },
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Prowess: whenever you cast a noncreature spell, +1/+1 EOT on this creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: prowess_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // ETB: return target instant or sorcery card from your graveyard to your hand.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![instant_or_sorcery_in_gy],
            }),
    )
}

fn prowess_pump(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn etb_return(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
