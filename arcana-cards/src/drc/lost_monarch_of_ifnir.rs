//! Lost Monarch of Ifnir — `{3}{B}` 4/4 Zombie Noble.
//! "Afflict 3." (keyword not in usable surface — GAP)
//! "Other Zombies you control have afflict 3." (static — GAP)
//! "At the beginning of your second main phase, if a player was dealt combat
//! damage by a Zombie this turn, mill three cards, then you may return a
//! creature card from your graveyard to your hand."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lost Monarch of Ifnir");
    let zombie = reg.interner_mut().intern("Zombie");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Afflict keyword is not in the usable keyword surface.
        ..Default::default()
    };

    // GAP: "Other Zombies you control have afflict 3" — keyword-granting static
    // for an unavailable keyword.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::PostCombatMain,
                whose: ControllerConstraint::You,
            },
            // GAP intervening-if: "if a player was dealt combat damage by a
            // Zombie this turn" — no script::* helper tracks combat damage by a
            // creature subtype this turn; left None.
            intervening_if: None,
            effect: second_main_mill,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn second_main_mill(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "then you may return a creature card from your graveyard to your
    // hand" — no non-targeted graveyard-creature-to-hand primitive (the
    // graveyard return effects require a chosen target id). The mill is
    // expressed.
    vec![Effect::Mill { player: trig.controller, count: 3 }]
}
