//! Xavier Sal, Infested Captain — `{B}{G}{U}` 3/3 Legendary Creature — Human Fungus Pirate.
//!
//! Oracle:
//! * {T}, Remove a counter from another permanent you control: Populate. (sorcery speed) — GAP'd
//! * {T}, Sacrifice another creature: Proliferate. (sorcery speed)
//!
//! The first ability has TWO inexpressible pieces: the cost "Remove a
//! counter from another permanent you control" (the engine's
//! `remove_self_counter` only removes from the source, not a chosen other
//! permanent) and the effect "Populate" (no `Effect::Populate` variant).
//! It is GAP'd whole. The second ability — `Sacrifice another creature`
//! (a chosen creature, `sacrifice_other`) paying for `Effect::Proliferate`
//! — is fully expressed.
//!
//! "Activate only as a sorcery" is a timing restriction; `is_instant_speed:
//! false` models sorcery-speed activation correctly.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xavier Sal, Infested Captain");
    let human = reg.interner_mut().intern("Human");
    let fungus = reg.interner_mut().intern("Fungus");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(fungus);
    subtypes.0.insert(pirate);

    let sac_filter = ObjectFilter::new().with_types(TypeLine::CREATURE.into());

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // Proliferate / Populate keyword tags are reminder-only here (no
        // standalone KeywordAbility variants); the abilities carry the rules.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "{T}, Remove a counter from another permanent you control: Populate." —
    //      cost (remove a counter from a CHOSEN other permanent) and effect (Populate)
    //      have no expressible primitive (remove_self_counter is source-only; no Effect::Populate).
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice another creature: Proliferate. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(sac_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: do_proliferate,
            }),
    )
}

fn do_proliferate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Proliferate]
}
