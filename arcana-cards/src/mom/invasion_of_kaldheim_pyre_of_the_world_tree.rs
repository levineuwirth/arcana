//! Invasion of Kaldheim // Pyre of the World Tree — `{3}{R}` red Battle — Siege with 6 defense.
//!
//! Front ETB: Exile all cards from your hand, then draw that many cards.
//!   Until the end of your next turn, you may play cards exiled this way.
//! Back face (Pyre of the World Tree — Enchantment):
//!   Discard a land card: This enchantment deals 2 damage to any target.
//!   Whenever you discard a land card, exile the top card of your library.
//!   You may play that card this turn.
//!
//! # GAPs
//! - ETB "exile all cards from your hand, then draw that many": no Effect for
//!   exile-hand-and-draw-same-count; the whole ETB effect is GAP'd.
//! - ETB "until end of your next turn, you may play exiled cards": permission
//!   replacement effect not expressible.
//! - defeat→cast-back-face not auto-wired (engine sends defeated Battle to GY).
//! - Back-face activated ability "Discard a land card: deal 2 damage to any target":
//!   back-face-only activated ability not modeled (face_gate mechanism deferred for
//!   transform-back; GAP per prompt).
//! - Back-face triggered ability "whenever you discard a land card, exile top card":
//!   back-face-only triggered ability not auto-installed (engine debt per prompt).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::Effect;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Kaldheim");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    // Back face: Pyre of the World Tree — Enchantment
    let back_name = reg.interner_mut().intern("Pyre of the World Tree");
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 6,
            })
            .with_transform_back(CardFace {
                name: back_name,
                characteristics: back_chars,
                spell_ability: None,
            })
            // GAP: defeat->cast-back-face not auto-wired
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_effect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_effect(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Exile all cards from your hand, then draw that many cards" —
    // no Effect for exile-own-hand-and-draw-equal-count; not expressible
    Vec::new()
}
