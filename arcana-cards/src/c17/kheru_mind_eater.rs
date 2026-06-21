//! Kheru Mind-Eater — `{2}{B}` 1/3 Vampire with Menace.
//! "Whenever this creature deals combat damage to a player, that player
//! exiles a card from their hand face down."
//! "You may look at cards exiled with this creature, and you may play
//! lands and cast spells from among those cards."
//!
//! Menace is a base keyword. The combat-damage trigger forces the
//! damaged player to exile a card from their OWN hand face down — there
//! is no Effect for "that player exiles a card of their choice from
//! their hand", so the trigger body is GAP'd (the trigger condition is
//! still wired so the catalog records the shape). The companion static
//! permission ("you may play those cards") is a continuous play-from-
//! exile permission with no expressible primitive — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Kheru Mind-Eater");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: gap_exile_from_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP (static): "You may look at cards exiled with this creature, and
    // you may play lands and cast spells from among those cards." A
    // continuous play-from-exile permission has no expressible primitive.
}

fn gap_exile_from_hand(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that player exiles a card from their hand face down" — no Effect
    // for forcing a player to exile a card of their choice from their own
    // hand (face down, linked to this source).
    Vec::new()
}
