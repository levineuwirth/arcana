//! Tergrid, God of Fright // Tergrid's Lantern
//!
//! Front face: Legendary Creature — God, {3}{B}{B}, Menace.
//! Whenever an opponent sacrifices a nontoken permanent or discards a permanent card,
//! you may put that card from a graveyard onto the battlefield under your control.
//! Back face: Legendary Artifact.
//! {T}: Target player loses 3 life unless they sacrifice a nonland permanent or discard a card.
//! {3}{B}: Untap Tergrid's Lantern.
//!
//! GAP: Front-face triggered ability "whenever an opponent sacrifices a nontoken permanent or
//! discards a permanent card, you may put that card from a graveyard onto the battlefield"
//! requires tracking opponent sacrifice/discard events and selecting the specific card — no
//! TriggerCondition variant covers opponent sacrifice of a permanent, and no Effect targets
//! a specific card from graveyard by tracking the sacrificed/discarded object. Omitted.
//! GAP: Back-face {T} activated ability "Target player loses 3 life unless they sacrifice a
//! nonland permanent or discard a card" — OptionalPaymentKind only supports Mana and Life;
//! a Sacrifice/Discard alternative cost is not expressible. Left GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tergrid, God of Fright");
    let god_sub = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Menace],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Tergrid's Lantern");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::ARTIFACT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Back face (Tergrid's Lantern): "{3}{B}: Untap Tergrid's Lantern."
            // Activatable only while the artifact back face is showing.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}: Untap Tergrid's Lantern.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: untap_self,
            }),
    )
}

fn untap_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
