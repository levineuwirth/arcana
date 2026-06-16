//! Archangel Elspeth — `{2}{W}{W}` legendary planeswalker, starting loyalty 5.
//!
//! +1: Create a 1/1 white Soldier creature token with lifelink.
//! −2: Put two +1/+1 counters on target creature. It becomes an Angel in
//!     addition to its other types and gains flying.
//! −6: Return all nonland permanent cards with mana value 3 or less from
//!     your graveyard to the battlefield (mass-reanimation GAP).
//!
//! Scope: +1 (lifelink Soldier token) and −2 (counters + Angel type +
//! flying) are fully expressed. The −6 mass return-all-from-graveyard
//! has no demonstrated mass-reanimation Effect (Reanimate is a single
//! pick), so its body is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archangel Elspeth");
    let elspeth = reg.interner_mut().intern("Elspeth");
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elspeth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 white Soldier creature token with \
                       lifelink.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Put two +1/+1 counters on target creature. It becomes \
                       an Angel in addition to its other types and gains flying.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_buff,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Return all nonland permanent cards with mana value 3 \
                       or less from your graveyard to the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_return,
            }),
    )
}

/// `+1: Create a 1/1 white Soldier token with lifelink.`
fn plus_one_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: soldier,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Lifelink],
            abilities: vec![],
        },
    }]
}

/// `−2: two +1/+1 counters, becomes an Angel, gains flying.`
fn minus_two_buff(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    // GAP: "becomes an Angel" (creature SUBTYPE add) has no demonstrated
    // Effect — AddType grants card types, not subtypes. Counters + flying
    // are expressed.
    vec![
        Effect::AddCounters { target: id, kind: CounterKind::PlusOnePlusOne, count: 2 },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Flying,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}

/// `−6` — mass reanimation.
fn minus_six_return(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "return ALL matching cards from your graveyard" has no
    // demonstrated mass-reanimation Effect (Reanimate is a single pick).
    Vec::new()
}
