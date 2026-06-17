//! Bladewing, Deathless Tyrant — `{5}{B}{R}` 6/6 Legendary Dragon
//! Skeleton with Flying and Haste.
//! "Whenever Bladewing deals combat damage to a player or planeswalker,
//! for each creature card in your graveyard, create a 2/2 black Zombie
//! Knight creature token with menace." (Planeswalker damage targeting is
//! approximated as player damage.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bladewing, Deathless Tyrant");
    let dragon = reg.interner_mut().intern("Dragon");
    let skeleton = reg.interner_mut().intern("Skeleton");
    // Pre-intern the token subtypes so the resolver's lookup succeeds.
    let _ = reg.interner_mut().intern("Zombie");
    let _ = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(skeleton);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: make_zombie_knights,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_zombie_knights(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::graveyard_matching(
        state,
        &ObjectFilter::creature(),
        trig.controller,
        trig.controller,
    );
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let knight = reg.interner().lookup("Knight").unwrap_or_default();
    let mut out = Vec::new();
    for _ in 0..n {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(zombie);
        subtypes.0.insert(knight);
        out.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: zombie,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![KeywordAbility::Menace],
                abilities: vec![],
            },
        });
    }
    out
}
