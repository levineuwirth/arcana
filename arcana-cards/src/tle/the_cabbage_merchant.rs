//! The Cabbage Merchant — `{2}{G}` 2/2 Legendary Human Citizen.
//! * "Whenever an opponent casts a noncreature spell, create a Food token." —
//!   wired as a SpellCast (opponent, noncreature) trigger minting a Food
//!   commodity token.
//! * "Whenever a creature deals combat damage to you, sacrifice a Food token."
//!   — wired as a DamageDealt (creature → player, combat) trigger; the resolver
//!   only acts when the damaged player is you, sacrificing one Food.
//! * "Tap two untapped Foods you control: Add one mana of any color." — the
//!   tap-two-Foods cost is wired, but "add one mana of any color" has no
//!   any-color AddMana primitive (only fixed-color ManaUnit::plain), so the
//!   mana payload is GAP'd.
//!
//! Food is parsed as a Scryfall keyword but is not a usable KeywordAbility
//! variant — the keyword line is empty; the Food token is minted by the trigger.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Cabbage Merchant");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);

    // "two untapped Foods you control" cost filter.
    let foods_to_tap = script::subtype_filter(reg, "Food")
        .controlled_by(ControllerConstraint::You)
        .untapped_only();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: make_food,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: sac_food_when_damaged,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap two untapped Foods you control: Add one mana of any color.".into(),
                cost: ActivationCost {
                    tap_other: Some(foods_to_tap),
                    tap_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_any_color,
            }),
    )
}

fn make_food(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}

fn sac_food_when_damaged(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Only "to you" — act when the damaged player is this card's controller.
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    if p != trig.controller {
        return Vec::new();
    }
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: script::subtype_filter(reg, "Food"),
        count: 1,
    }]
}

fn add_any_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Add one mana of any color." — no any-color AddMana primitive (only
    // fixed-color ManaUnit::plain is available). The tap-two-Foods cost is wired.
    Vec::new()
}
