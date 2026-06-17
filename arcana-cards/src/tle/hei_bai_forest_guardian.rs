//! Hei Bai, Forest Guardian — `{3}{G}` 4/4 Legendary Bear Spirit.
//! "When Hei Bai enters, reveal cards from the top of your library until you reveal
//!  a Shrine card. You may put that card onto the battlefield. Then shuffle."
//! "{W}{U}{B}{R}{G},{T}: For each legendary enchantment you control, create a 1/1
//!  colorless Spirit creature token …"

use arcana_core::effects::{Effect, DigRest, RevealDest, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hei Bai, Forest Guardian");
    let bear = reg.interner_mut().intern("Bear");
    let spirit = reg.interner_mut().intern("Spirit");
    // Pre-intern subtype strings used by resolvers.
    let _shrine = reg.interner_mut().intern("Shrine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_reveal_shrine,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{U}{B}{R}{G}, {T}: For each legendary enchantment you control, create a 1/1 colorless Spirit creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_spirits,
            }),
    )
}

fn etb_reveal_shrine(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (fidelity): "You may" — RevealUntil deterministically puts the found
    // Shrine onto the battlefield.
    let filter = script::subtype_filter(reg, "Shrine");
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter,
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: None,
    }]
}

fn make_spirits(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::ENCHANTMENT.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY))
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, ctx.controller);
    let spirit = reg.interner().lookup("Spirit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    // GAP: the token's "This token can't block or be blocked by non-Spirit
    // creatures" ability is not attached to the minted token.
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: spirit,
                colors: ColorSet::colorless(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect()
}
