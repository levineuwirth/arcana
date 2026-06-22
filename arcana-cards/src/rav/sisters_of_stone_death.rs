//! Sisters of Stone Death — `{4}{B}{B}{G}{G}` 7/5 Legendary Creature — Gorgon.
//!
//! * {G}: Target creature blocks Sisters of Stone Death this turn if able.
//!     (GAP: "must block [this creature] if able" has no `Effect` primitive —
//!      `ForbidBlocking`/`Goad` are the inverses; omitted.)
//! * {B}{G}: Exile target creature blocking or blocked by Sisters of Stone
//!   Death.  (Wired via `TargetFilter::CreatureBlockingOrBlockedBySource`.)
//! * {2}{B}: Put a creature card exiled with Sisters of Stone Death onto the
//!   battlefield under your control.
//!     (GAP: no primitive returns a card "exiled with this source" — the
//!      exile-set linkage isn't an exposed accessor; omitted.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sisters of Stone Death");
    let gorgon = reg.interner_mut().intern("Gorgon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gorgon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}{G}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{G}: Exile target creature blocking or blocked by \
                       Sisters of Stone Death."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::CreatureBlockingOrBlockedBySource,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_combatant,
            }),
    )
}

fn exile_combatant(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}
