//! Ratonhnhaké꞉ton — `{W}{U}{B}` 3/3 Legendary Human Assassin.
//! Conditional static hexproof + can't-be-blocked (GAP).
//! Whenever it deals combat damage to a player, create a 1/1 black Assassin
//! token with menace. (Reflexive Equipment return/attach is GAP'd.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ratonhnhaké꞉ton");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "As long as Ratonhnhaké꞉ton hasn't dealt damage yet, it has
    // hexproof and can't be blocked" — a conditional static keyed on a
    // has-dealt-damage state is not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // No "self deals combat damage" trigger exists; DamageDealt's
                // source_filter is the only path. Scope to a Legendary creature
                // source (this card is legendary) — the established self-
                // approximation; a true self-only scope is engine debt.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: make_assassin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_assassin(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let assassin = reg.interner().lookup("Assassin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assassin);
    // GAP: reflexive "When you do, return target Equipment card from your
    // graveyard to the battlefield, then attach it to that token" — attaching
    // to the just-created token id is not expressible.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: assassin,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Menace],
            abilities: vec![],
        },
    }]
}
