//! Gate to Tumbledown — nonbasic land — Mountain Gate.
//! "({T}: Add {R}.)" / "Gate to Tumbledown enters the battlefield
//! tapped." / "{3}{R}, {T}: Seek a nonland card. Activate only once."
//! Enters tapped + a red mana ability. GAP: "Seek" is an
//! Arena/Alchemy-only mechanic with no engine primitive, and
//! "Activate only once" (per game) has no `ActivationCost` field — the
//! seek activation's effect is empty.

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
    let name = reg.interner_mut().intern("Gate to Tumbledown");
    let mountain = reg.interner_mut().intern("Mountain");
    let gate = reg.interner_mut().intern("Gate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mountain);
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
                text: "{T}: Add {R}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}, {T}: Seek a nonland card. Activate only once.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}").expect("valid cost"),
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

fn add_red_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn seek_nonland(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Seek a nonland card" — Seek is an Arena/Alchemy-only
    // mechanic with no engine primitive.
    // GAP: "Activate only once." — once-per-game activation limits are
    // not expressible.
    Vec::new()
}
