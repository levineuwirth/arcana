//! Valduk, Keeper of the Flame — `{2}{R}` 3/2 legendary red Human Shaman. "At the
//! beginning of combat on your turn, for each Aura and Equipment attached to Valduk,
//! create a 3/1 red Elemental creature token with trample and haste. Exile those tokens
//! at the beginning of the next end step."
//! GAP: "count attached Auras/Equipment" not in engine; using count_matching as
//! approximation (counts all permanents with equipment/aura types you control).
//! Token exile at end step via CreateTokenSacEot.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valduk, Keeper of the Flame");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let _elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_combat_create_elementals,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_combat_create_elementals(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: count of attached Auras/Equipment not directly accessible;
    // using count of enchantments+artifacts you control as approximation
    let aura_count = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ENCHANTMENT.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let equip_count = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let n = aura_count + equip_count;
    if n == 0 {
        return Vec::new();
    }
    let elemental = reg.interner().lookup("Elemental")
        .expect("Elemental interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateTokenSacEot { controller: trig.controller, token: token.clone() })
        .collect()
}
