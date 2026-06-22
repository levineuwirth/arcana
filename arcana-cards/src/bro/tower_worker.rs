//! Tower Worker — `{3}` 1/3 Artifact Creature — Assembly-Worker with
//! Reach. "{T}: Add {C}. If you control creatures named Mine Worker and
//! Power Plant Worker, add {C}{C}{C} instead."
//!
//! Wired: the tap mana ability producing one colorless mana. The
//! conditional "{C}{C}{C} instead" upgrade depends on controlling two
//! specifically-named creatures; a mana ability must resolve to a fixed
//! AddMana so the conditional upgrade is GAP'd (base {C} is emitted).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tower Worker");
    let worker = reg.interner_mut().intern("Assembly-Worker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(worker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: conditional "{C}{C}{C} instead" when controlling
                // creatures named Mine Worker and Power Plant Worker — a
                // mana ability resolves to a fixed AddMana, so only the
                // base {C} is produced.
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless,
            }),
    )
}

fn add_colorless(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}
