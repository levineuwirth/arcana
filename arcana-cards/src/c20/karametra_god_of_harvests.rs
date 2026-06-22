//! Karametra, God of Harvests — `{3}{G}{W}` 6/7 Legendary Enchantment
//! Creature — God. Indestructible.
//! "As long as your devotion to green and white is less than seven,
//! Karametra isn't a creature." — a devotion-gated continuous static
//! removing the creature type; GAP'd (no expressible devotion-gated type
//! removal).
//! "Whenever you cast a creature spell, you may search your library for a
//! Forest or Plains card, put it onto the battlefield tapped, then shuffle."
//! — wired as a SpellCast (creature filter) trigger that tutors a card whose
//! subtype is Forest OR Plains onto the battlefield tapped.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karametra, God of Harvests");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Indestructible],
        // GAP: "As long as your devotion to green and white is less than
        // seven, Karametra isn't a creature." — devotion-gated continuous
        // type removal is not expressible.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: search_forest_or_plains,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn search_forest_or_plains(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest = reg.interner().lookup("Forest");
    let plains = reg.interner().lookup("Plains");
    let mut subtypes = Vec::new();
    if let Some(f) = forest {
        subtypes.push(f);
    }
    if let Some(p) = plains {
        subtypes.push(p);
    }
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .with_subtypes_any(subtypes),
        tapped: true,
    }]
}
