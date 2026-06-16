//! Lilah, Undefeated Slickshot — `{1}{U}{R}` 3/3 Legendary Creature — Human
//! Rogue (red/blue).
//!
//! Oracle text:
//! * Prowess (Whenever you cast a noncreature spell, this creature gets +1/+1
//!   until end of turn.) — `KeywordAbility::Prowess` is NOT in the usable
//!   keyword surface, so it is not emitted on the keyword line. Instead the
//!   prowess behavior is decomposed into a triggered ability: whenever you cast
//!   a noncreature spell, pump this creature +1/+1 until end of turn.
//! * Whenever you cast a multicolored instant or sorcery spell from your hand,
//!   exile that spell instead of putting it into your graveyard as it resolves;
//!   if you do, it becomes plotted. — GAP: this is a resolution-time
//!   replacement plus the Plot mechanic; there is no Effect variant for
//!   "exile-instead-of-graveyard-and-make-plotted", and the `Plot` keyword is
//!   not in the usable surface.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
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
    let name = reg.interner_mut().intern("Lilah, Undefeated Slickshot");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
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
            // Prowess, decomposed into a triggered ability (the keyword itself
            // is not in the usable surface).
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
            }),
        // GAP: "Whenever you cast a multicolored instant or sorcery spell from
        // your hand, exile that spell instead of putting it into your graveyard
        // as it resolves. If you do, it becomes plotted." — no Effect variant
        // for the exile-instead-of-graveyard replacement combined with the Plot
        // mechanic; Plot is also not in the usable keyword surface.
    )
}

/// Prowess: when you cast a noncreature spell, this creature gets +1/+1 until
/// end of turn.
fn prowess_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
