//! He Who Hungers — `{4}{B}` 3/2 Legendary Creature — Spirit with Flying and
//! Soulshift 4.
//!
//! * Flying.
//! * `{1}, Sacrifice a Spirit: Target opponent reveals their hand. You choose a
//!   card from it. That player discards that card. Activate only as a sorcery.`
//!   ("You choose the card" falls back to opponent-chooses — DiscardChoice has
//!   no controller-chooses-from-revealed-hand variant; minor fidelity gap.)
//! * Soulshift 4 (the engine synthesizes the dies-return ability from the
//!   keyword).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("He Who Hungers");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let spirit_filter = script::subtype_filter(reg, "Spirit");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Soulshift(4)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{1}, Sacrifice a Spirit: Target opponent reveals their hand. You
            // choose a card from it. That player discards that card. Activate
            // only as a sorcery."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice a Spirit: Target opponent reveals their hand. You choose a card from it. That player discards that card. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice_other: Some(spirit_filter),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: targeted_discard,
            }),
    )
}

fn targeted_discard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    // GAP: "you choose a card from it" — DiscardChoice has no
    // controller-chooses-from-revealed-hand variant; fall back to opponent
    // choosing the discarded card.
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::OpponentChooses,
    }]
}
