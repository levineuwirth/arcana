//! Golden Sidekick — `{W}{B}` 1/3 Enchantment Creature — Bat Glimmer
//! with Flying and Lifelink.
//! Whenever you gain life, a random creature card in your hand
//! perpetually gets +X/+X, where X is the amount of life you gained.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Golden Sidekick");
    let bat = reg.interner_mut().intern("Bat");
    let glimmer = reg.interner_mut().intern("Glimmer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    subtypes.0.insert(glimmer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_life_gained,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_life_gained(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "a random creature card in your hand perpetually gets +X/+X"
    // — no effect to apply a perpetual buff to a (random) card in hand,
    // and no accessor for the life-gained amount on a LifeGained
    // trigger. Emitting no effect.
    Vec::new()
}
