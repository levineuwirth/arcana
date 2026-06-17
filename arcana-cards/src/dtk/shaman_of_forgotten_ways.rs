//! Shaman of Forgotten Ways — `{2}{G}` 2/3 Human Shaman.
//! "{T}: Add two mana in any combination of colors. Spend this mana only to
//! cast creature spells." and "Formidable — {9}{G}{G}, {T}: Each player's life
//! total becomes the number of creatures they control. Activate only if
//! creatures you control have total power 8 or greater."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shaman of Forgotten Ways");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: first ability "{T}: Add two mana in any combination of colors. Spend
    // this mana only to cast creature spells." — Effect::AddMana needs fixed
    // colors and there is no spend-restriction tag, so it is not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{9}{G}{G}, {T}: Each player's life total becomes the number of creatures they control.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{9}{G}{G}").expect("valid cost"),
                tap: true,
                // GAP: "Activate only if creatures you control have total power
                // 8 or greater" — no documented total-power activation_condition.
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: set_life_to_creature_count,
        }),
    )
}

fn set_life_to_creature_count(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::all_players(state)
        .into_iter()
        .map(|p| {
            let n = script::count_matching(
                state,
                &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                p,
            );
            let _ = &ctx;
            Effect::SetLifeTotal { player: p, amount: n }
        })
        .collect()
}
