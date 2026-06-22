//! Apothecary White — `{3}{W}` 3/4 Legendary Human Cleric with Vigilance.
//!
//! Oracle:
//! * Vigilance (keyword).
//! * Whenever you attack, you create a Food token for each player being
//!   attacked.
//! * {W}, {T}, Tap X untapped Foods you control: Create X 1/1 white Human
//!   creature tokens.
//!
//! Notes:
//! * "Food" in the Scryfall keyword line is a token-type indicator, not a
//!   `KeywordAbility` variant, so only Vigilance is in the keyword vec.
//! * The attack trigger creates a Food per player being attacked — a dynamic
//!   count with no script accessor for "number of players being attacked", so
//!   the per-player scaling effect is GAP'd (whole effect omitted rather than
//!   hardcoding a literal).
//! * The activated ability taps Food(s) you control as a cost via `tap_other`
//!   and creates Human token(s). The variable "X" (tap X Foods → create X
//!   Humans) is not expressible — `tap_other_count`/token count are fixed — so
//!   it is modeled as tapping one Food to create one Human, with the X-scaling
//!   GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Apothecary White");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let food = reg.interner_mut().intern("Food");
    // Token subtype (Human) for the activated ability's token.
    let _human_token = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    // Food filter for the tap-other activation cost.
    let food_filter = ObjectFilter::new().with_subtype_sym(food);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_make_food,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, {T}, Tap X untapped Foods you control: Create X 1/1 white Human creature tokens.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    tap: true,
                    tap_other: Some(food_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_human,
            }),
    )
}

fn attack_make_food(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "a Food token for each player being attacked" — no script accessor
    // for the number of players being attacked; dynamic count not computable.
    Vec::new()
}

fn make_human(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: variable "X" (tap X Foods → create X Humans) — tap_other_count and
    // token count are fixed; modeled as tap one Food → create one Human.
    let human = reg.interner().lookup("Human").unwrap_or_default();
    let mut token_subs = SubtypeSet::default();
    token_subs.0.insert(human);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: human,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subs,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
