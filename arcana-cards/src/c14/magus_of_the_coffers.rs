//! Magus of the Coffers — `{4}{B}` 4/4 black Human Wizard.
//! "{2}, {T}: Add {B} for each Swamp you control."

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Magus of the Coffers");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let _swamp_filter = ObjectFilter::new().with_subtypes_any(vec![swamp]);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}: Add {B} for each Swamp you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_black_per_swamp,
            }),
    )
}

fn add_black_per_swamp(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let swamp = reg.interner().lookup("Swamp").expect("Swamp interned");
    let swamp_filter = ObjectFilter::new().with_subtypes_any(vec![swamp]);
    let n = script::count_matching(state, &swamp_filter, ctx.controller);
    if n == 0 {
        return Vec::new();
    }
    let mana = (0..n).map(|_| ManaUnit::plain(ManaColor::Black, ctx.source)).collect();
    vec![Effect::AddMana { player: ctx.controller, mana }]
}
