//! Cloudpost — nonbasic land, subtype Locus.
//! "This land enters tapped." and "{T}: Add {C} for each Locus on the
//! battlefield." Enters-tapped via `EntersWithSpec::Tapped`; the dynamic
//! mana amount counts every Locus on the battlefield (any controller) at
//! resolution.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cloudpost");
    let locus = reg.interner_mut().intern("Locus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(locus);
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
                text: "{T}: Add {C} for each Locus on the battlefield."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_per_locus,
            }),
    )
}

fn add_colorless_per_locus(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "for each Locus on the battlefield" — any controller, so no
    // controlled_by constraint on the filter.
    let filter = script::subtype_filter(reg, "Locus");
    let n = script::count_matching(state, &filter, ctx.controller);
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Colorless, ctx.source);
            n as usize
        ],
    }]
}
