//! Thought-Stalker Warlock — `{2}{B}` 2/2 Lizard Warlock.
//!
//! Menace.
//! When this creature enters, choose target opponent. If they lost life
//! this turn, they reveal their hand, you choose a nonland card from it,
//! and they discard that card. Otherwise, they discard a card.
//!
//! Both branches end in the targeted opponent discarding one card, so the
//! resolver always emits that discard (a faithful floor). The "if they
//! lost life this turn, reveal + you choose a nonland card" refinement is
//! GAP'd: there is no targeted/choose-from-revealed-hand discard
//! primitive, so the opponent's own choice (`ControllerChooses` for the
//! discarding player) stands in for both branches.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thought-Stalker Warlock");
    let lizard = reg.interner_mut().intern("Lizard");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_target_opponent_discards,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

fn etb_target_opponent_discards(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    // GAP: the "if they lost life this turn, reveal hand + you choose a
    // nonland card" branch isn't expressible (no targeted/choose-from-
    // revealed-hand discard); both branches collapse to one discard.
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
