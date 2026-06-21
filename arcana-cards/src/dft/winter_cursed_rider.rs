//! Winter, Cursed Rider — `{U}{B}` 3/2 Legendary Human Warlock.
//! "Ward—Pay 2 life.
//!  Artifacts you control have \"Ward—Pay 2 life.\"
//!  Exhaust — {2}{U}{B}, {T}, Exile X artifact cards from your graveyard:
//!  Each other nonartifact creature gets -X/-X until end of turn. (Activate
//!  each exhaust ability only once.)"
//!
//! Ward—Pay 2 life is a non-mana ward cost with no expressible KeywordAbility
//! (only mana-cost Ward is supported) — GAP. The artifact-anthem static is a
//! keyword-granting buff with no hook — GAP. The Exhaust activation models
//! only its mana + tap cost; the "Exile X artifact cards from your graveyard"
//! variable cost has no field, the dynamic-X -X/-X sweep cannot read that X,
//! and "activate only once (ever)" has no exact once-ever field — so the
//! effect body is GAP'd.

// GAP (keyword): "Ward—Pay 2 life" — non-mana ward not expressible.
// GAP (static): "Artifacts you control have Ward—Pay 2 life" — no
// keyword-granting anthem hook.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Winter, Cursed Rider");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Exhaust — {2}{U}{B}, {T}, Exile X artifact cards from your graveyard: Each other nonartifact creature gets -X/-X until end of turn."
                .into(),
            // GAP: "Exile X artifact cards from your graveyard" variable cost
            // has no exile-from-graveyard field; "activate only once (ever)"
            // (Exhaust) has no exact once-ever field. Mana + tap modeled.
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}{B}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: exhaust_sweep,
        }),
    )
}

fn exhaust_sweep(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Each other nonartifact creature gets -X/-X until end of turn"
    // where X = artifact cards exiled from graveyard — the variable X is not
    // readable here (no exile-X cost field to source it from).
    Vec::new()
}
