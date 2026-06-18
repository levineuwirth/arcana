//! Doorkeeper — `{1}{U}` 0/4 Creature — Homunculus with Defender.
//!
//! Oracle:
//! * Defender.
//! * `{2}{U}, {T}: Target player mills X cards, where X is the number of
//!   creatures you control with defender.`
//!
//! The Defender keyword is a base characteristic. The activated ability's
//! shape (mana + tap cost, target player) is wired, but its mill amount X
//! scales off "creatures you control WITH DEFENDER" — the `script::`/
//! `ObjectFilter` surface exposes no keyword-membership predicate, so the
//! dynamic count is uncomputable. Per the dynamic-amount rule we GAP the whole
//! effect rather than emit a wrong fixed mill. (Scryfall's "Mill" keyword tag
//! is descriptive, not a real `KeywordAbility` variant — only Defender is
//! emitted in the keyword line.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doorkeeper");
    let homunculus = reg.interner_mut().intern("Homunculus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homunculus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}, {T}: Target player mills X cards, where X is the number of creatures you control with defender.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mill_x,
            }),
    )
}

fn mill_x(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X = "number of creatures you control with DEFENDER". No keyword
    // predicate exists on ObjectFilter / script (the refinement set has types,
    // colors, cmc, power/toughness, subtypes, supertypes, tap, token — but no
    // keyword-membership filter), so the dynamic mill amount is uncomputable.
    // Emitting a fixed literal would be a materially wrong card, so the whole
    // effect is GAP'd.
    Vec::new()
}
