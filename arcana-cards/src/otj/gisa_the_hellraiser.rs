//! Gisa, the Hellraiser — `{3}{B}{B}` 4/4 Legendary Human Warlock.
//! "Ward—{2}, Pay 2 life."
//! "Skeletons and Zombies you control get +1/+1 and have menace."
//! "Whenever you commit a crime, create two tapped 2/2 blue and black
//!  Zombie Rogue creature tokens. This ability triggers only once each turn."
//!
//! GAP: Ward—{2}, Pay 2 life is a mixed mana+life ward cost; KeywordAbility::Ward
//! takes a pure mana cost only, so this non-mana ward is not expressible.
//! GAP: "Whenever you commit a crime" has no matching TriggerCondition variant.
//!
//! The Skeletons/Zombies anthem IS wired: a `SelfEntersBattlefield` trigger
//! installs a `filtered_pump` (+1/+1, layer 7c) plus a `filtered_keyword`
//! (Menace, layer 6) over creatures you control whose subtype is Skeleton OR
//! Zombie (`with_subtypes_any`), anchored to Gisa with
//! `Duration::WhileSourceOnBattlefield`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gisa, the Hellraiser");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let zombie = reg.interner_mut().intern("Zombie");
    let _ = (skeleton, zombie); // interned so the resolver can look them up
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    // GAP: "Ward—{2}, Pay 2 life." — mixed mana+life ward, not a pure
    // mana cost; not expressible as KeywordAbility::Ward.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Whenever you commit a crime, create two tapped Zombie Rogue
    // tokens (once each turn)." — no crime TriggerCondition variant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_skeleton_zombie_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "Skeletons and Zombies you control get +1/+1 and have menace",
/// anchored to Gisa and lasting until she leaves the battlefield.
fn install_skeleton_zombie_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let skeleton = reg
        .interner()
        .lookup("Skeleton")
        .expect("Skeleton interned during register()");
    let zombie = reg
        .interner()
        .lookup("Zombie")
        .expect("Zombie interned during register()");
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![skeleton, zombie]);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                filter.clone(),
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                filter,
                KeywordAbility::Menace,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
