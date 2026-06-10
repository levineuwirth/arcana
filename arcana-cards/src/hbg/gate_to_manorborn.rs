//! Gate to Manorborn — nonbasic land — Forest Gate.
//! "({T}: Add {G}.)" "Gate to Manorborn enters the battlefield
//! tapped." and "{3}{G}, {T}: Seek a nonland card. Activate only
//! once."
//! Enters-tapped Gate with a green mana ability.
//! GAP: 'Seek a nonland card' — Seek is an Arena-only mechanic with no
//! Effect variant; the activation resolves as a no-op.
//! GAP: 'Activate only once' (once per game) — no once-ever activation
//! limiter in the catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gate to Manorborn");
    let forest = reg.interner_mut().intern("Forest");
    let gate = reg.interner_mut().intern("Gate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(forest);
    subtypes.0.insert(gate);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}, {T}: Seek a nonland card. Activate only once.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: seek_nonland,
            }),
    )
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn seek_nonland(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'Seek a nonland card' — Arena-only mechanic, no Effect
    // variant; 'Activate only once' (once per game) also has no
    // catalog limiter.
    Vec::new()
}
