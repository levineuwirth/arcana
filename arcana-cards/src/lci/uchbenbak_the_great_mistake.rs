//! Uchbenbak, the Great Mistake — `{3}{U}{B}` 6/4 Legendary Skeleton Horror.
//! Vigilance, menace.
//! Descend 8 — {4}{U}{B}: Return this card from your graveyard to the
//! battlefield with a finality counter on it. Activate only if there are
//! eight or more permanent cards in your graveyard and only as a sorcery.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Uchbenbak, the Great Mistake");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Descend 8 — {4}{U}{B}: Return this card from your graveyard to the battlefield with a finality counter on it. Activate only if there are eight or more permanent cards in your graveyard and only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{U}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: descend_reanimate,
        }),
    )
}

fn descend_reanimate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return THIS card from your graveyard to the battlefield with a
    // finality counter" — no self-targeting graveyard-return Effect is
    // available (ReturnFromGraveyardToBattlefield needs a chosen target id;
    // Reanimate is filter-based, not self). The Descend-8 precondition
    // (8+ permanent cards in graveyard) and sorcery-speed restriction also
    // have no activation_condition builder in the demonstrated API.
    Vec::new()
}
