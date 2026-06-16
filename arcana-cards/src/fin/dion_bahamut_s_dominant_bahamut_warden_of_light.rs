//! Dion, Bahamut's Dominant // Bahamut, Warden of Light
//!
//! Front face: Legendary Creature — Human Noble Knight, 3/3, {3}{W}
//!   Dragonfire Dive — During your turn, Dion and other Knights you control have flying.
//!     GAP: "during your turn" continuous conditional not expressible as static keyword.
//!     GAP: Dragonfire Dive keyword not in KeywordAbility enum.
//!   When Dion enters, create a 2/2 white Knight creature token.
//!   {4}{W}{W}, {T}: Exile Dion, then return it to the battlefield transformed.
//!     Activate only as a sorcery.
//!     (Modeled as exile-then-return-to-battlefield-transformed via ExilePermanent +
//!      ReturnFromExileToBattlefield — gap: transform on return not expressible via
//!      DelayedAction; use Transform directly after exile workaround.)
//!     GAP: exile-then-return-transformed is not directly expressible; modeled as
//!      Effect::Transform only.
//!
//! Back face: Legendary Enchantment Creature — Saga Dragon, Flying
//!   I, II — Wings of Light — Put a +1/+1 counter on each other creature you control.
//!     Those creatures gain flying until end of turn.
//!   III — Gigaflare — Destroy target permanent. Exile Bahamut, then return it to the
//!     battlefield (front face up).
//!     GAP: "return it to the battlefield front face up" is Transform again; modeled as
//!      Effect::Transform after exile/return.
//!   GAP: back-face-only triggered abilities (Saga chapter triggers) not auto-installed
//!     on transform.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dion, Bahamut's Dominant");

    let sub_human = reg.interner_mut().intern("Human");
    let sub_noble = reg.interner_mut().intern("Noble");
    let sub_knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_human);
    subtypes.0.insert(sub_noble);
    subtypes.0.insert(sub_knight);

    // Pre-intern Knight for token creation
    let _knight2 = reg.interner_mut().intern("Knight");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Bahamut, Warden of Light — Legendary Enchantment Creature — Saga Dragon
    let back_name = reg.interner_mut().intern("Bahamut, Warden of Light");
    let back_sub_saga = reg.interner_mut().intern("Saga");
    let back_sub_dragon = reg.interner_mut().intern("Dragon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_sub_saga);
    back_subtypes.0.insert(back_sub_dragon);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB trigger: create a 2/2 white Knight creature token
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_knight,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {4}{W}{W}, {T}: Transform Dion (activate only as sorcery)
            // GAP: exact "exile then return transformed" not expressible; use Transform directly.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{W}{W}, {T}: Exile Dion, then return it to the battlefield transformed. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{W}{W}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_to_back,
            })
            // GAP: back-face-only triggered abilities (Saga chapter triggers I, II, III)
            // not auto-installed on transform.
    )
}

fn etb_create_knight(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight").expect("Knight interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(knight);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: knight,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn transform_to_back(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: oracle says "exile Dion, then return it to the battlefield transformed";
    // Transform handles the flip directly without exile/return cycle.
    vec![Effect::Transform { target: ctx.source }]
}
