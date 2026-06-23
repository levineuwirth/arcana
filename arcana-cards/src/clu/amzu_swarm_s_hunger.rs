//! Amzu, Swarm's Hunger — `{3}{B}{G}` 3/3 Legendary Insect Shaman with
//! Flying and Menace.
//! "Other Insects you control have menace." (static — NOW WIRED via an
//! ETB-installed `ContinuousEffect::filtered_keyword`.)
//! "Whenever one or more cards leave your graveyard, you may create a
//! 1/1 black and green Insect creature token, then put a number of
//! +1/+1 counters on it equal to the greatest mana value among those
//! cards. Do this only once each turn." (no cards-leave-graveyard
//! trigger — GAP)
//!
//! Note: the static reads "OTHER" Insects; `filtered_keyword` matches base
//! characteristics, so Amzu (an Insect) self-includes — a documented minor
//! fidelity gap. Amzu already has Menace as a base keyword, so this is inert
//! on the source and faithful on the rest of your Insects.

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
    let name = reg.interner_mut().intern("Amzu, Swarm's Hunger");
    let insect = reg.interner_mut().intern("Insect");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: trigger — "Whenever one or more cards leave your graveyard, …" No
    // cards-leave-graveyard TriggerCondition variant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_insect_menace,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "Insects you control have menace", anchored to Amzu.
fn etb_install_insect_menace(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let insect = reg
        .interner()
        .lookup("Insect")
        .expect("Insect interned during register()");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You)
                .with_subtype_sym(insect),
            KeywordAbility::Menace,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
