//! Blurry Beeble — `{U}` 1/1 Artifact Creature — Cyborg Beeble.
//!
//! * Blurry (Un-set evasion joke keyword) — GAP: not a modeled
//!   `KeywordAbility`; the "wearing glasses" block-restriction is not
//!   expressible.
//! * Whenever this creature deals combat damage to a player, you may
//!   draw a card. If you do, discard a card.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blurry Beeble");
    let cyborg = reg.interner_mut().intern("Cyborg");
    let beeble = reg.interner_mut().intern("Beeble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyborg);
    subtypes.0.insert(beeble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: loot_on_combat_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn loot_on_combat_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may draw a card. If you do, discard a card." — draw-then-discard.
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}
