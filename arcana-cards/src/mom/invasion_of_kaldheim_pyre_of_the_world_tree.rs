//! Invasion of Kaldheim // Pyre of the World Tree — `{3}{R}` red Battle — Siege with 6 defense.
//!
//! Front ETB: Exile all cards from your hand, then draw that many cards.
//!   Until the end of your next turn, you may play cards exiled this way.
//! Back face (Pyre of the World Tree — Enchantment):
//!   Discard a land card: This enchantment deals 2 damage to any target.
//!   Whenever you discard a land card, exile the top card of your library.
//!   You may play that card this turn.
//!
//! Defeat→back-face is auto-wired by the engine SBA. The back-face activated
//! ability ("Discard a land card: deal 2 damage to any target") is wired,
//! face-gated to the enchantment face (1), with the discard-cost restricted to a
//! land card. The front ETB and the back-face discard-a-land trigger are GAP'd
//! below.
//!
//! # GAPs
//! - ETB "exile all cards from your hand, then draw that many": no Effect for
//!   exile-hand-and-draw-same-count; the whole ETB effect is GAP'd.
//! - ETB "until end of your next turn, you may play exiled cards": permission
//!   replacement effect not expressible.
//! - Back-face triggered ability "whenever you discard a land card, exile top
//!   card; you may play it this turn": `TriggerCondition::CardDiscarded` has no
//!   discarded-card filter, so it can't be restricted to land cards; not wired.

use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::targets::{ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement};
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
            // Front (Siege) ETB — GAP'd (see module docs), face-gated to 0.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_effect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            // Back (Pyre of the World Tree): "Discard a land card: ~ deals 2
            // damage to any target." Discard cost restricted to a land card;
            // face-gated to the enchantment face (1).
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a land card: This enchantment deals 2 damage to any target.".into(),
                cost: ActivationCost {
                    discard_other: Some(ObjectFilter::new().with_types(TypeLine::LAND.into())),
                    discard_other_count: 1,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: back_deal_two,
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

fn back_deal_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 2,
    }]
}
